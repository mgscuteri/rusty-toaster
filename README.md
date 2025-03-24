## About

The goal of this project is to provide an all around solution for hosting a photo website on a low budget machine.

### Included

- Rust webserver with endpoints for
  - /albums - lists out all of the folders in a "photos" folder
  - /albums/{album} - lists out all of the photos in an album
  - /albums/{album}/{photo} - serves a specific photo
  - /static/{file} - serves a static ui
- Angular UI
  - Integrates with the Rust webserver
  - Displays thumbnails for all of the albums
  - Click on a thumbnail to see the rest of the photos
  - Allows copy url of album to share direct links to albums

### Hardware

I've deployed it to this modest 2 bay NAS, where it runs quite happily.

- Asustor Driverstor 2 PRO
- Realtek RTD1296 Quad-Core 1.4 GHz CPU
- Uses 2 GB DDR4 – 40% more efficient
- htps://www.asustor.com/en/product?p_id=72
