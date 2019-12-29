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

Benchmark all:

```sh
$ cd euler/euler-go/euler
$ go test -bench=.
```

Benchmakr one:

```sh
$ cd euler/euler-go/euler
$ go test euler14_test.go euler14.go -bench=.
```

(有点反常，不能直接在根目录下执行 `go test`，必须进入到测试文件所在的目录执行 `go test`...而且在测试中不是使用 assert 等断言语句，而是用 t.Error()，t.Fatal() 等方法来表示失败)

Format code:

```sh
$ cd euler/euler-go
$ gofmt -w ./euler
```

## How to Write Test

See example [euler/euler14_test.go](./euler/euler14_test.go)

Refs:

- [testing - 单元测试](https://books.studygolang.com/The-Golang-Standard-Library-by-Example/chapter09/09.1.html)
- [testing - 基准测试](https://books.studygolang.com/The-Golang-Standard-Library-by-Example/chapter09/09.2.html)
