#! /usr/bin/env bash

kind delete cluster
kind create cluster

session_name="demo"

if tmux has-session -t="$session_name" 2> /dev/null; then
  tmux attach -t "$session_name"
  exit
fi

# crud
tmux new -d -s "$session_name" -c "$(pwd)"
tmux rename-window -t "$session_name" "crud"
tmux split-window -d

tmux send-keys -t "$session_name:crud.0" "cd deployment-crud" enter C-l "vim src/main.rs" enter
tmux send-keys -t "$session_name:crud.1" "cd deployment-crud" enter C-l "watch kubectl get po,deploy" enter

# watcher
tmux new-window -t "$session_name" -n "watcher"
tmux split-window -d

tmux send-keys -t "$session_name:watcher.0" "cd pod-watcher" enter C-l "vim src/main.rs" enter
tmux send-keys -t "$session_name:watcher.1" "cd pod-watcher" enter C-l "kubectl get po -Aw"


# reflector
#
tmux new-window -t "$session_name" -n "reflector"
tmux split-window -d

tmux send-keys -t "$session_name:reflector.0" "cd pod-reflector" enter C-l "vim src/main.rs" enter
tmux send-keys -t "$session_name:reflector.1" "cd pod-reflector" enter C-l "curl -S localhost:3000 | jq ."


# controller
tmux new-window -t "$session_name" -n "controller"
tmux split-window -d

tmux send-keys -t "$session_name:controller.0" "cd website/src" enter C-l "ls" enter
tmux send-keys -t "$session_name:controller.1" "cd website" enter C-l "watch kubectl get website,svc,deploy,po"

tmux attach -t "$session_name:crud"
