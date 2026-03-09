### Locations:

- Locations are what the name suggests; they are places you can move in and out of
- Locations can contain sub-locations and so on and so forth
    - Locations can access information of their sub-locations, for example, for describing the surroundings
- Locations are given parse commands to handle and do things with
- How should locations be handled? I think using a setup_locations fn that sets up and adds all the logic seems appropriate, or maybe sub-classes of Location such as House, etc, etc, that are only used once, but help keep the logic clean.