### relay-rs 

The idea is to relay the data from the input stream to the output stream with some stats like the speed of reads and writes. Currently, the input stream can be stdin or a file. Likewise, the output stream can be stdout or a file. But looking forward to turning the streams generic such that we can even plug a WebSocket/TCP connection or any streams which exhibit read/write behavior.
