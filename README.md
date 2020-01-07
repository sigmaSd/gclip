# gclip
Crossplatfrom xclip script (with caveat)

## Usage
    // copy data from stdin to clipboard
    echo "hello world" | gclip
    // copy data from file to clipboard
    gclip file
    
 ## Caveat
  gclip will spawn a gtk button btn, that you have to press inorder to actually copy the data
  
 ## Why
  It works on wayland
