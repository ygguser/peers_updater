### Yggrasil network peers checker / updater

The utility is designed to check the availability of peers and automatically update them in the Yggdrasil configuration file, as well as using the admin API - addPeer method.

By setting up the automatic launch of the utility on a schedule, you can forget that public peers sometimes stop working and you need to update them manually.

```
Usage: peers_updater [OPTIONS]

Options:
  -p, --print           Print the peers sorted by latency. When using this parameter, all other parameters will be ignored.
  -c, --config <FILE>   The path to the Yggdrasil configuration file [default: /etc/yggdrasil.conf or C:\ProgramData\Yggdrasil\yggdrasil.conf]
  -u, --update_cfg      Make changes to the Yggdrasil configuration file. If not specified, no changes will be made to the file.
  -a, --api             Add/remove peers during execution (requires enabling the admin API)
  -n, --number <VALUE>  The number of peers to add (excluding extra ones) [default: 3]
  -e, --extra <VALUE>   A space-separated string with the URIs of the peers that should always be in the configuration
  -i, --ignore <VALUE>  A space-separated string of characters. Peers whose URIs contain combinations of characters will not be added to the configuration
  -r, --restart         Restart the Yggdrasil (systemd or windows) service
  -h, --help            Print help information
  -V, --version         Print version information
```

To simply display a list of peers sorted by response time, use the `-p` parameter. At the same time, all other parameters will be ignored, no changes will be made to the configuration of Yggrasil.

In order for the utility to work fully and correctly, making changes to the Yggdrasil settings, the user with whose rights it is launched must have the appropriate permissions to change the configuration file and/or use the Admin API.

It doesn't make sense to use the `-r` (restart Yggdrasil) and `-a` (use admin API) flags at the same time.

#### Usage Examples

Output of a sorted list of peers:

```
./peers_updater -p
```

Updating peers in the configuration file at the specified path (two peers will be added):

```
sudo ./peers_updater -c /home/user/tst/yggdrasil.conf -n 2 -u
```

Updating peers (2 peers will be added) in the configuration file with the default path to it, as well as adding peers using the admin API:

```
sudo ./peers_updater -n 2 -u -a
```

Updating peers (2 peers will be added) in the configuration file with the default path to it, and adding additional peers (in a space-separated line). A total of 4 peers will be added:

```
sudo ./peers_updater -n 2 -u -a -e "tcp://my.favorite.peer.uk:7777 tls://i.love.uk:7777"
```

Updating peers (1 peer will be added). At the same time the peers will be ignored, in the URI of which there are: "tls:/ badpeer unstable.peer.far.away.from.me certain.port.peer.co:6767 377337":

```
sudo ./peers_updater -n 1 -u -i "tls:/ badpeer unstable.peer.far.away.from.me certain.port.peer.co:6767 7337"
```

The utility can be run on a schedule using cron (Linux) or using another scheduler (Windows).

##### Example with scheduled launch
Updating the configuration file on a schedule probably makes sense no more than once a week. Because there is little chance that 2-3-4 peers specified in the configuration file will stop working at the same time during the week.

Launching the editor:
```
sudo crontab -e
```

At the end of the file, add: 
```
0 0 * * 0 /path/peers_updater -u -n 3 -r -c /etc/yggdrasil.conf >/dev/null 2>&1
```
Save the changes.

Now the peer nodes will be updated on Sundays at 0 o'clock.

#### Build from source

The project is being built without errors and warnings with cargo 1.65.0 and rustc 1.65.0.

```
git clone https://github.com/ygguser/peers_updater
cd peers_updater
cargo buid --release
```
