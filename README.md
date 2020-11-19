# AdminSploit
This is a remote access tool/backdoor written in rust.  
It uses Discord webhooks to transfer files to the client.

![alt text](https://github.com/H3nr1X/AdminSploit/blob/main/showcase.png?raw=true)


______________________________________________________________________________

# Current features  

Reverse tcp socket    
Low antivirus detection  
Remote Command Prompt execution  
Remote Powershell execution  
Screen capture  
Webcam capture  
Directory traversal  
File deletion  
________________________________________________________________________________

# Upcoming features

256-bit end-to-end encryption  
File dowloading from Google Drive  
Changing wallpaper  
Execution on startup  
Disabling Command Prompt and Task Manager  
Message sending  

_________________________________________________________________________________

# Current issues

Capturing a screen causes a memory leak  
Webcam capture may not work and crash the connection  
Certain invalid commands from user-end may crash the command-handler thread on the server, and the user has to reconnect  
