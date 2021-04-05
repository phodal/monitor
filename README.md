# monitor

setup from: https://www.waveshare.com/wiki/9.7inch_e-Paper_HAT

size: 1200 × 825 

```
tar zxvf bcm2835-1.xx.tar.gz
cd bcm2835-1.xx
./configure
make
sudo make check
sudo make install
```

run demo code:

```
tar zxvf IT8951.tar.gz
cd IT8951
make clean
make
sudo ./IT8951 0 0 01.bmp
```

## Tasks

```
https://graph.microsoft.com/v1.0/me/todo/lists/
```


see in: [https://docs.microsoft.com/en-us/graph/api/todotasklist-list-tasks?view=graph-rest-1.0&tabs=http](https://docs.microsoft.com/en-us/graph/api/todotasklist-list-tasks?view=graph-rest-1.0&tabs=http)

Task by id

```
https://graph.microsoft.com/v1.0/me/todo/lists/AQMkADAwATM0MDAAMS04N2E2LWRhMDItMDACLTAwCgAuAAADlaHHnFnn1UuoFE2pMt0j5QEAaMraZPkN_0mltv0IMNqe5wAD5QRmOQAAAA==/tasks
```
