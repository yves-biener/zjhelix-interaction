# Zellij - Helix interaction

I created this plugin to allow any application to communicate with a running helix application, i.e. to open a file for me, enter a macro, etc. As helix is not server based you cannot pipe commands to open for example a file. Instead zellij is used to send corresponding keystrokes to simulate user interaction in helix. With this you can essentially send any abritrary command to helix (or any application you want).

The plugin makes heavy use of pipe messages to create corresponding actions which shall be executed. Hence the message needs the following parameter:
- *name*: name of the pane which shall receive the command
- *payload*: string of the user input that shall be simulated

The plugin will try to find the named pane, switch to it and write the payload followed by a carriage return.

## Example

```sh
zellij pipe --name Helix -- "oHello World!"
```

The example will write "Hello World!" to your active running Helix session (given that you are already in Normal Mode).
