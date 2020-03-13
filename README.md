# xfer-server
Rust based TCP File transfer - Server

Very simple program that listens for TCP connections from clients and files. 

The filename is first received, and **overwrites** any existing file of the same name! The file is received in chunks and the complete file is *assumed* to be received and the write is finished.

## Warning!

This is currently just a test project, and is missing features such as:

* Alternative option if file exists
* Authentiction (which clients can "connect" to you)
* Hash check to ensure correct file is received
* Encryption for file transfers