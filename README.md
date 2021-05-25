# bad_rocket_fileserver
An html/http-accessable file server.

I use this on my desktop to host a server which has a browser-based interface for downloading static files. It's not very good, but it is what I am using, and I believe in backing up your code.

This contains a PKGBUILD and a systemd service file. The service file needs editing in order to share the right directory -- I was lazy and didn't put configuration inside /etc like I should have.
