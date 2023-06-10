# wifi-cli-tool
A simple cli tool to get your wifi info, check ping and connect to network or disconnect.

```terminal
>> wifi-cli.exe
Enter an option
1) Get info
2) Connect
3) Disconnect
4) Get ping
5) Check selected hosts ping
6) Check selected ips
0) Exit
> 3
Wifi was disconnected.

Enter an option
1) Get info
2) Connect
3) Disconnect
4) Get ping
5) Check selected hosts ping
6) Check selected ips
0) Exit
> 1

There is 1 interface on the system:

    Name                   : Wi-Fi
    Description            : -
    GUID                   : -
    Physical address       : -
    Interface type         : Primary
    State                  : disconnected
    Radio status           : Hardware On
                             Software On

    Hosted network status  : Not available




Enter an option
1) Get info
2) Connect
3) Disconnect
4) Get ping
5) Check selected hosts ping
6) Check selected ips
0) Exit
> 2
Enter the name of your wifi
> some-wifi
Enter your password
> *************
Can't connect.

Enter an option
1) Get info
2) Connect
3) Disconnect
4) Get ping
5) Check selected hosts ping
6) Check selected ips
0) Exit
> 4
Enter count of iteration
> 6
Ping(0) - 61.570112ms
Ping(1) - 44.474212ms
Ping(2) - 58.971662ms
Ping(3) - 71.034168ms
Ping(4) - 52.88785ms
Ping(5) - 49.650037ms
>

Enter an option
1) Get info
2) Connect
3) Disconnect
4) Get ping
5) Check selected hosts ping
6) Check selected ips
0) Exit
> 5
Enter host
> rust-lang.org goggle.com reddit.com
Enter count of iteration
> 5
No such host is known.(goggle.com)
Ping(0) - 56.22215ms
Ping(1) - 58.523875ms
Ping(2) - 61.529062ms
Ping(3) - 54.727212ms
Ping(4) - 54.6978ms
Working hosts: rust-lang.org reddit.com
>

Enter an option
1) Get info
2) Connect
3) Disconnect
4) Get ping
5) Check selected hosts ping
6) Check selected ips
0) Exit
> 6
Enter host
> 127.0.0.1 3.4.5.6
Enter count of iteration
> 5
No ping elements.(3.4.5.6)
Ping(0) - 47.1µs
Ping(1) - 92.8µs
Ping(2) - 38.2µs
Ping(3) - 51.2µs
Ping(4) - 31.5µs
Working hosts: 127.0.0.1
>

Enter an option
1) Get info
2) Connect
3) Disconnect
4) Get ping
5) Check selected hosts ping
6) Check selected ips
0) Exit
> 0
```
