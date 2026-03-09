### Locations:

- Locations are what the name suggests; they are places you can move in and out of

- Locations can contain sub-locations and so on and so forth
    - Locations can access information of their sub-locations, for example, for describing the surroundings

- Locations are given parse commands to handle and do things with

- How should locations be handled? I think using a setup_locations fn that sets up and adds all the logic seems appropriate, or maybe sub-classes of Location such as House, etc, etc, that are only used once, but help keep the logic clean. These bug-classes could be given the command, and they could send back a response to the GameMaster which changes the current location, etc. etc. etc.
    - These honestly don't need to be too complex. I can imagine something like this:
        ```rust
        pub struct Window {
            let parent: Location,
            let broken: bool,
            let covered: bool
        }
        ```
        and some helper function, like help(), and a parse() that takes in keywords and other things; i.e. the inventory, etc, and runs internal functions that process stuff and run functions on the inventory, which has callbacks to maybe print things when something is destroyed, etc.

        And the House can go to self.window and check things, and print house-specific things about the window, namely that it is infernally *bright*

        Of course the window is a location too that has an internal state, etc etc etc. Nothing of the house affects you as you stare at the window, right? Or perhaps to print that your eyes feel better you need to inform the house-parent that its window is now covered, ah the confusion!