### Locations:

- Locations are what the name suggests; they are places you can move in and out of

- Locations can contain sub-locations and so on and so forth
    - Locations can access information of their sub-locations, for example, for describing the surroundings

- Locations are given parse / summarize commands to handle and do things with. a location can call this on its current_location field (which stores which sub location yer in), and children can access their parent location and tell it to do things and such.

- Basic outline of a Location:
    - a current_location field which is a named enum of that location's states, which sub-location you're currently in.
    
    - some helper function, like help(), and parse() and summarize() fns too.

- The GameState I currently have should be replaced with a World() location. And it summarizes where you are, just like; 
    - You are on a large outcropping, cliffs, etc, etc.
    - and this would get called whenever you can see it, maybe not if you're inside or don't know where you are yet.
    - and has sub locations for itself like house, random travel points, etc.