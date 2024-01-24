# Goop's middle name CLI
A simple CLI to choose one of Goop's middle names and add more to the server!

## Requirments
* Linux or MacOS *(untested)*
* A POSIX compliant shell *(ex: bash, zsh)*
* Rust with Cargo to build

## How to install/build
* Clone the repository
```git clone https://github.com/eocow/goops_middle_name.git```
* Open the root foler ```goops_middle_name```
* Open your shell here and run ```chmod +x install.sh```
* Run ```install.sh```
  * You may need to input your sudo password
* Thats it!

## Who is Goop?
An orange cat with a lot of middle names

## How to add names
*Anyone can add names, though only those on the local network can add to the server*
* To add names to your local list (stored in ```~/.config/goop/names```) run:
```goop add <name>```
* To push names to the server run:
  ```goop push```
  *   A backup copy of each new list of names is saved on the server to allow for reversions.

## ToDo
* ```pull``` function from the server
* server syncing with the github list of names