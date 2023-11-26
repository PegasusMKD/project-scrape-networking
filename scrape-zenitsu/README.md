# Scrape Zenitsu

The UDP server (and main "game master") for Project: Scrape.

## Purpose

This server will be responsible for any-and-all communications during game instances, it will be the de-facto "game master" for any game-related events.
For example, collision-detection, bullet creation, weapon selection, destroying of unused/useless entities (ex. bullet which colided with a surface should be destroyed), player camera rotations, etc.

## Architecture

Uses 2 "servers", or to be more specific threads, 1 for inbound communication and 1 for outbound communication and state processing.
To be more precise, the inbound thread receives some UDP packets, it then stores them into a message queue.
Then, once every tick (16ms) the outbound thread copies over the message queue and empties it out. After getting a copy, it 
starts updating the state of all the objects according to the messages which it has received, and of course, any objects which aren't 
related to the client sending signals.
