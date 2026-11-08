use std::net::{IpAddr, Shutdown, SocketAddr, TcpStream, ToSocketAddrs};
use std::string::String;
use std::sync::Arc;
use std::time::{Duration, Instant};

use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{DigitallySignedStruct, SignatureScheme};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
struct NoCertificateVerification {
    provider: Arc<rustls::crypto::CryptoProvider>,
}

impl ServerCertVerifier for NoCertificateVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls12_signature(
            message,
            cert,
            dss,
            &self.provider.signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(
            message,
            cert,
            dss,
            &self.provider.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.provider
            .signature_verification_algorithms
            .supported_schemes()
    }
}

//#[derive(Debug)]
pub struct Peer {
    pub uri: String,
    pub addr: String,
    pub port: String,
    pub proto: String,
    pub region: String,
    pub country: String,
    pub is_alive: bool,
    pub latency: u128,
}

impl Peer {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        uri: String,
        addr: String,
        port: String,
        proto: String,
        region: String,
        country: String,
        is_alive: bool,
        latency: u128,
    ) -> Self {
        Peer {
            uri,
            addr,
            port,
            proto,
            region,
            country,
            is_alive,
            latency,
        }
    }

    pub fn set_latency(&mut self) {
        self.is_alive = false;
        self.latency = u128::MAX;

        //QUIC
        if self.proto.eq_ignore_ascii_case("quic") {
            self.set_quic_latency();
            return;
        }

        //TCP
        let addr = match format!("{}:{}", self.addr, self.port)
            .to_socket_addrs()
            .ok()
            .and_then(|mut addrs| addrs.next())
        {
            Some(addr) => addr,
            None => return,
        };

        let now = Instant::now();

        let stream = match TcpStream::connect_timeout(&addr, CONNECT_TIMEOUT) {
            Ok(stream) => stream,
            Err(_) => return,
        };

        self.latency = now.elapsed().as_millis();
        self.is_alive = true;

        let _ = stream.shutdown(Shutdown::Both);
    }
        
    
    fn set_quic_latency(&mut self) {
        let host = self
            .addr
            .trim_start_matches('[')
            .trim_end_matches(']');

        let host_port = match host.parse::<IpAddr>() {
            Ok(IpAddr::V6(_)) => format!("[{}]:{}", host, self.port),
            _ => format!("{}:{}", host, self.port),
        };

        let addrs: Vec<SocketAddr> = match host_port.to_socket_addrs() {
            Ok(addrs) => addrs.collect(),
            Err(_) => return,
        };

        if addrs.is_empty() {
            return;
        }

        let runtime = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(runtime) => runtime,
            Err(_) => return,
        };

        let provider = Arc::new(rustls::crypto::ring::default_provider());

        let tls_config = rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(NoCertificateVerification {
                provider,
            }))
            .with_no_client_auth();

        let quic_tls = match quinn::crypto::rustls::QuicClientConfig::try_from(tls_config) {
            Ok(config) => config,
            Err(_) => return,
        };

        let client_config = quinn::ClientConfig::new(Arc::new(quic_tls));

        for addr in addrs {
            let bind_addr: SocketAddr = if addr.is_ipv4() {
                "0.0.0.0:0".parse().unwrap()
            } else {
                "[::]:0".parse().unwrap()
            };

            let hostname = host.to_string();
            let start = Instant::now();

            let result = runtime.block_on(async {
                tokio::time::timeout(CONNECT_TIMEOUT, async {
                    let mut endpoint =
                        quinn::Endpoint::client(bind_addr).map_err(|e| e.to_string())?;

                    endpoint.set_default_client_config(client_config.clone());

                    let connecting = endpoint
                        .connect(addr, &hostname)
                        .map_err(|e| e.to_string())?;

                    let connection = connecting.await.map_err(|e| e.to_string())?;

                    let (_send, _recv) =
                        connection.open_bi().await.map_err(|e| e.to_string())?;

                    let elapsed = start.elapsed();

                    connection.close(0u32.into(), b"probe complete");
                    endpoint.close(0u32.into(), b"probe complete");

                    Ok::<Duration, String>(elapsed)
                })
                .await
            });

            match result {
                Ok(Ok(elapsed)) => {
                    self.latency = elapsed.as_millis();
                    self.is_alive = true;
                    return;
                }
                Ok(Err(_)) | Err(_) => continue,
            }
        }
    }
}
