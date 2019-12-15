# Euler by Go

The project is initiated by following commands:

```sh
$ cd euler
$ mkdir euler-go && cd euler-go
$ go mod init euler-go
```

## How to Run Test

Test all:

```sh
$ cd euler/euler-go/euler
$ go test -v
```

or:

```sh
$ cd euler/euler-go
$ go test euler-go/euler -v
```

Test one:

```sh
$ cd euler/euler-go/euler
$ go test euler1_test.go euler1.go -v
```

(有点反常，不能直接在根目录下执行 `go test`，必须进入到测试文件所在的目录执行 `go test`...而且在测试中不是使用 assert 等断言语句，而是用 t.Error()，t.Fatal() 等方法来表示失败)
