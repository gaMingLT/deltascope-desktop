
# Deltascope Desktop

This repository contains a desktop based implementation of the original ``Deltascope`` application which can be found [here](https://github.com/gaMingLT/deltascope). 
This implementation is focused on importing most of the original functionality of the application to the desktop and expanding on it.

## Reasoning

The reason for reimplementing the original application in to a Desktop based version is because I wanted to make use of the Rust programming language and the [Tauri](https://tauri.app/) framework. as the original frontend could be 'easily' repurposed as the Tauri framework supports the usage of a Nextjs application as the frontend.

## Running

The application can be started by installing all the required packages for a Tauri & Reactjs application. With all packages installed the following command can be execute: ``pnpm tauri dev``, which will started dev version of the application.

## Problems

Currently the application can only be run in dev mode, building the application into a releasable binary results in the css layout not applying correctly. The reason is that the Tauri frameworks requires static HTML files to be present, but this functionality is in a kind of limbo state because of the current update to a new Nextjs 13 version. Some investigation has been performed to identified and rectify the problem but so far without success
