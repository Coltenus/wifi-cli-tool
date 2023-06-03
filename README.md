# wifi-cli-tool
A simple cli tool to get your wifi info and connect to network or disconnect.

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
google.com - 31.0367ms
youtube.com - 32.7049ms
reddit.com - 21.9248ms
github.com - 30.0343ms
Avarage - 28.925175ms
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
> github.com reddit.com rust-lang.com
github.com - 29.5696ms
reddit.com - 21.797525ms
rust-lang.com - 17.358325ms
Avarage - 22.908483ms
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
127.0.0.1 - 72.9µs
No ping elements.(3.4.5.6)
Avarage - 72.9µs
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