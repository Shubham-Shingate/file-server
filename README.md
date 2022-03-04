# File Server description
Create a server socket to receive data from the client. Once the connection is established properly then server is ready to receive data. Otherwise the connection is terminated.
Then we have to handle the data sent by client so we create threads and spawn the threads, if size of the data is matched then we process the data. Otherwise we terminate the connection.
Once the data is processed by the server then we terminate the connection and flush the stream.

# File Server 
- Please note that the file client will be cloned within the Docker image from a separate repository [here](https://github.com/Shubham-Shingate/file-client)
## Commands to Build and Run Project
- Please note that the Dockerfile within this repository (File Server) will copy the contents from the *second* repository (File Client) into the created Docker image.
### Build Docker Image
Terminal$ `sudo docker build -t [tagname] .`
- Note, change 'tagname' to whatever you want to name it

### Run Docker Image
Terminal$ `sudo docker run -it [tagname]`