# Pub/Sub in Rust

A simple pub/sub implementation in rust

## Protocol

Our protocol is very simple, we have two possible events:

- **Configure event**: An event that configures the connection to assigne it to a queue.
  
We pass the queue name as the first argument, then, after the `:` we say whether it is a publisher or a consumer 

```
<queue>:<"pub" | "sub">
```

- **Message event**: An event that can only be sent from producers (a write from a connection that was configured as a consumer will do nothing) that broadcasts the message to all consumers

This one is even simpler just send the data over in bytes, you don't need to specify the queue

```
<data goes here>
```

## Usage

Checkout this code snippet written in go:

```go
package main

import (
	"log/slog"
	"net"
	"os"
	"time"
)

// This function will be used to connect producers
func ConnectProducer() net.Conn {
  // Dial the server via TCP
	producer, err := net.Dial("tcp", "localhost:8080")
	if err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}
  // Send a write to configure the connection
	_, err = producer.Write([]byte("queue:pub"))
	if err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}
	return producer
}

// This function will be used to connect consumers
func ConnectConsumer() net.Conn {
  // Dial the server via TCP
	consumer, err := net.Dial("tcp", "localhost:8080")
	if err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}
  // Send a write to configure the connection
	_, err = consumer.Write([]byte("queue:sub"))
	if err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}
	return consumer
}

func main() {
	producer := ConnectProducer()
	defer producer.Close()

	consumer := ConnectConsumer()
	defer consumer.Close()

	go func() {
		for {
      // Read to all messages being broadcasted
			buffer := make([]byte, 1024)
			n, err := consumer.Read(buffer)
			if err != nil {
				panic(err)
			}
			println(string(buffer[:n]))
		}
	}()

	for {
    // Write from producer every one second
		_, err := producer.Write([]byte("Hello, World!"))
		if err != nil {
			panic(err)
		}
		time.Sleep(1 * time.Second)
	}
}
```
