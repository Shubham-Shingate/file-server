# File Server 
- Please note that the file client will be cloned within the Docker image from a separate repository [here](https://github.com/Shubham-Shingate/file-client)
## Commands to Build and Run Project
- Please note that the Dockerfile within this repository (File Server) will copy the contents from the *second* repository (File Client) into the created Docker image.
### Build Docker Image
Terminal$ `sudo docker build -t [tagname] .`
- Note, change 'tagname' to whatever you want to name it

### Run Docker Image
Terminal$ `sudo docker run -it [tagname]`