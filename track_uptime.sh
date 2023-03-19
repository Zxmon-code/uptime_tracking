#!/bin/bash

filename="$(date +%Y_%m_%d)"
track_time() {
    while [[ true ]]; do
      echo $(hyprctl activewindow | grep -o "class: \S*" | grep -o "\S*\$") >> "$HOME/uptime/${filename}"  
      sleep 60;
    done
}

track_time
