#!/bin/bash

filename="$(date +%Y_%m_%d)"
track_time() {
    while [[ true ]]; do
      if [[ "$(hyprctl cursorpos)" != "$(cat $HOME/uptime/$filename | tail -n10 | head -n1)" ]];
      then
        if [ "$(hyprctl activewindow | grep -o 'class: .*')" != "class: obsidian" ];
        then
          # echo $(hyprctl activewindow | grep -o "class: \S*" | grep -o "\S*\$") >> "$HOME/uptime/${filename}"  
          echo $(hyprctl cursorpos) >> "$HOME/uptime/${filename}"
          sleep 60;
        fi
      fi
    done
}

track_time
