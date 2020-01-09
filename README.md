# gclip
Crossplatfrom xclip script (with caveat)

## Usage
    // copy data from stdin to clipboard
    echo "hello world" | gclip
    // copy data from file to clipboard
    gclip file
    
 ## Caveat
  gclip will spawn a gtk Label saying Copied! that you have to dismess (ctrlc, or x button)
  
 ## Why
  It works on wayland
