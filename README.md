## MBC - The Modbus CLI

A simple CLI for issuing modbus requests. Supports modbus and modbus over TCP.


### Usage

To view full help information, run:

```bash
mbc --help
```


For example, to read the first 10 coil statuses of a Modbus/TCP server on 127.0.0.1:
```bash
$ mbc 'tcp://127.0.0.1' read coils 0 10
address status
0       false
1       false
2       true
3       true
4       false
5       true
6       false
7       false
8       false
9       false
```