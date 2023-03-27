# imagenav
A very simple image navigator that keeps a control channel open on
stdin. Usefull for exploring images on a romote box over ssh.

Only tested on linux as of today.

## usage

Accepts a directory containing images as input. It *should* filter out
non-image files. No recursion is done into sub-directories.

	imagenav ~/dir/photos/
	
## commands

Currently supported commands are

	* `->` (arrow right) next image
    * `<-` (arrow left) previous image
	* `f`  fullscreen
	* `r`  rotate
    * `q`  quit
