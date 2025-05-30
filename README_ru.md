### Yggrasil network peers checker / updater

[![Статус сборки](https://github.com/ygguser/peers_updater/actions/workflows/Build+Release.yml/badge.svg)](https://github.com/ygguser/peers_updater/actions/workflows/Build+Release.yml) 
[![Участники](https://img.shields.io/github/contributors/ygguser/peers_updater?label=%D0%A3%D1%87%D0%B0%D1%81%D1%82%D0%BD%D0%B8%D0%BA%D0%B8)](https://github.com/ygguser/peers_updater/graphs/contributors)
[![Лицензия: MIT](https://img.shields.io/github/license/ygguser/peers_updater?label=%D0%9B%D0%B8%D1%86%D0%B5%D0%BD%D0%B7%D0%B8%D1%8F)](/LICENSE)

Утилита предназначена для проверки доступности пиров и автоматического их обновления в конфигурационном файле [Yggdrasil](https://yggdrasil-network.github.io/), а так же, с помощью метода admin API - addPeer.

Настроив автоматический запуск утилиты по расписанию, можно забыть о том, что публичные пиры иногда перестают работать и нужно их обновлять вручную.

```
Использование: peers_updater [ОПЦИИ]

Опции:
  -p, --print           Вывод отсортированного по задержке доступа списка пиров
  -c, --config <FILE>   Путь к конфигурационному файлу Yggdrasil [по-умолчанию: /etc/yggdrasil/yggdrasil.conf или C:\ProgramData\Yggdrasil\yggdrasil.conf]
  -u, --update_cfg      Вносить изменения в конфигурационный файл. Если не указано, изменения в файл вноситься не будут.
  -a, --api             Добавлять/удалять пиры с помощью Admin API (требуется включение  admin API в настройках)
  -n, --number <VALUE>  Количество пиров, которое будет автоматически добавлено (без учета дополнительных пиров) [по-умолчанию: 3]
  -e, --extra <VALUE>   Разделенная пробелами строка с URI пиров, которые всегда должны быть в конфигурационном файле
  -i, --ignore <VALUE>  Разделенная пробелами строка символов. Пиры, в URI которых встречаются сочетания символов, не будут добавлены в конфигурацию
  -I, --ignore_country <VALUE> Разделенная пробелами строка с названиями стран, которые не будут добавляться в конфигурацию
  -r, --restart         Перезапускать сервис Yggdrasil (systemd или windows)
  -S, --self_update     Самообновление этой утилиты. Исполняемый файл будет загружен из релизов на GitHub (если там опубликована более новая версия), и текущая версия будет заменена новой.
  -h, --help            Вывод этой справки
  -V, --version         Вывод версии
```

Чтобы просто вывести список пиров, отсортированный по времени отклика используйте параметр `-p`. При этом, никаких изменений в конфигурацию Yggrasil внесено не будет. 

Для того, чтобы утилита могла полноценно и корректно работать, внося измения в настройки Yggdrasil, у пользователя, с правами которого она запускается, должны быть соответствующие разрешения на изменение конфигурационного файла и/или использование Admin API.

Флаги `-r` (перезапустить Yggdrasil) и `-a` (использовать admin API) не имеет смысла использовать одновременно.

**Обратите внимание:** опции `-i` (`--ignore`) и `-I` (`--ignore_country`) нужно использовать осмысленно. Дело в том, что разработчики Yggdrasil рекомендуют использовать 2-3 географически ближайших к вам публичных пира для соединения с сетью. Географически ближайших - для того, чтобы задержка соединения была минимальной и соединение было более стабильным.

[peer_updater](https://github.com/ygguser/peers_updater) автоматически выбирает узлы с наименьшим временем отклика, но используя упомянутые выше параметры можно [случайно] проигнорировать ближайшие к вам пиры и таким образом увеличить нагрузку на свой узел и снизить качество соединения.

#### Примеры использования

Вывод отсортированного списка пиров:

```
./peers_updater -p
```

Обновление пиров в конфигурационном файле по указанному пути (будет занесено два пира):

```
sudo ./peers_updater -c /home/user/tst/yggdrasil.conf -n 2 -u
```

Обновление пиров (будут добавлены 2 пира) в конфигурационном файле с путём к нему по-умолчанию, а так же, добавление пиров с использованием admin-API:

```
sudo ./peers_updater -n 2 -u -a
```

Обновление пиров (будут добавлены 2 пира) в конфигурационном файле с путём к нему по-умолчанию, а так же, добавление дополнительных пиров (в строке разделенной пробелами). Всего будет добавлено 4 пира:

```
sudo ./peers_updater -n 2 -u -a -e "tcp://my.favorite.peer.ru:7777 tls://i.love.ru:7777"
```

Обновление пиров (будет добавлен 1 пир). При этом, будут игнорироваться пиры, в URI которых встречаются: `tls:/ badpeer unstable.peer.su certain.port.peer.co:6767 1337`:

```
sudo ./peers_updater -n 1 -u -i "tls:/ badpeer unstable.peer.su certain.port.peer.co:6767 1337"
```

По расписанию утилиту можно запускать с помощью cron (Linux) или с помощью другого планировщика (Windows). 

##### Пример с запуском по расписанию
Обновлять конфигурационный файл по расписанию, вероятно, имеет смысл не чаще, чем раз в неделю. Т.к., мала вероятность того, что 2-3-4 пира, указанных в конфигурационном файле, одновременно перестанут работать в течении недели.

Запускаем редактор:
```
sudo crontab -e
```

В конец файла добавляем:
```
0 0 * * 0 /path/peers_updater -u -n 3 -r -c /etc/yggdrasil/yggdrasil.conf >/dev/null 2>&1
```
Сохраняем изменения.

Теперь обновление пиров будет происходить по воскресеньям в 0 часов.

#### Где скачать утилиту

Ссылки для скачивания peers_updater под нужную архитектуру доступны на странице [релизов](https://github.com/ygguser/peers_updater/releases).

<!--- https://doc.rust-lang.org/nightly/rustc/platform-support.html --->

| Имя сборки	| Описание |
| -- | -- |
| aarch64-unknown-linux-gnu |	ARM64 Linux (kernel 4.1, glibc 2.17+) |
| i686-pc-windows-gnu |	32-bit MinGW (Windows 8+) |
| i686-unknown-linux-gnu |	32-bit Linux (kernel 3.2+, glibc 2.17+) |
| x86_64-pc-windows-gnu	| 64-bit MinGW (Windows 8+) |
| x86_64-unknown-linux-gnu |	64-bit Linux (kernel 3.2+, glibc 2.17+) |
| arm-unknown-linux-gnueabi	| ARMv6 Linux (kernel 3.2, glibc 2.17) |
| armv7-unknown-linux-gnueabihf	| ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17) |
| x86_64-apple-darwin | 64-bit macOS (10.7+, Lion+) |

#### Сборка из исходников

Проект собирается без ошибок и предупреждений с cargo 1.80 и rustc 1.80.

Просто установите rust, `git` (для Linux возможно понадобится `gcc-multilib`) и выполните следующее:

```
git clone https://github.com/ygguser/peers_updater
cd peers_updater
cargo build --release
```

<details><summary>Дополнительные возможности</summary>

##### Настройка функционала при сборке

По-умолчанию, проект соберется со всем описанным выше функционалом, однако есть возможность отключить ненужные вам функции и тем самым немного снизить размер исполняемого файла.

Пример: 

```
cargo build --release --no-default-features --features "update_cfg self_updating"
```

Возможные значения параметра features:

- `updating_cfg` - обновление конфигурационного файла Yggdrasil
- `using_api` - использование API для обновления пиров
- `self_updating` - возможность самообновления

Так выглядит справка по параметрам программы, собранной с опцией `--no-default-features`:

```
Usage: peers_updater [OPTIONS]

Options:
  -p, --print                   Print the peers sorted by latency. When using this parameter, all other parameters will be ignored.
  -i, --ignore <VALUE>          A space-separated string of characters. Peers whose URIs contain combinations of this characters will not be added to the configuration
  -I, --ignore_country <VALUE>  A space-separated string containing the names of countries that will not be added to the configuration
  -h, --help                    Print help
  -V, --version                 Print version
```
</details>
