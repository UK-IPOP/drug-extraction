FROM golang:1.17

WORKDIR /go-app

COPY . .

CMD ["go", "test", "-bench=.", "--benchtime",  "100000x"] 