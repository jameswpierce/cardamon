document.addEventListener("DOMContentLoaded", () => {
  ui.init();
});

const ui = {
  init: () => {
    const elements = {
      artists: document.getElementById("artists").getElementsByTagName("li"),
      albums: document.getElementById("albums").getElementsByTagName("li"),
      tracks: document.getElementById("tracks").getElementsByTagName("li"),
      queue: document.getElementById("queue"),
      play: document.getElementById("play"),
      next: document.getElementById("next"),
      previous: document.getElementById("prev"),
      repeat: document.getElementById("repeat"),
      shuffle: document.getElementById("shuffle"),
      audio: document.getElementById("audio"),
      nowPlaying: document.getElementById("now-playing"),
    };

    const handleTrackChange = (track) => {
      elements.audio.src = track.filePath;
      elements.nowPlaying.innerText = `${track.name} - ${track.artist} - ${track.album}`;
      console.log(track);
    };

    const handlePlay = (track) => {
      elements.play.innerText = "Pause";
    };

    const handlePause = (track) => {
      elements.play.innerText = "Play";
    };

    const player = Player.init({
      audioElement: elements.audio,
      queue: Array.from(elements.tracks).map((track) => {
        return {
          id: track.id,
          ...track.dataset,
        };
      }),
      onTrackChange: handleTrackChange,
      onPlay: handlePlay,
      onPause: handlePause,
    });

    console.log(player.queue);

    const firstTrack = elements.tracks[0].dataset;
    handleTrackChange(firstTrack);

    elements.play.addEventListener("click", async () => {
      player.isPlaying() ? await player.pause() : await player.play();
    });

    elements.next.addEventListener("click", async () => {
      await player.next();
    });

    elements.previous.addEventListener("click", async () => {
      await player.previous();
    });

    elements.repeat.addEventListener("click", () => {
      switch (player.repeatState) {
        case player.repeatStates().NONE:
          player.repeatState = player.repeatStates().ONE;
          player.repeat(player.repeatStates().ONE);
          break;
        case player.repeatStates().ONE:
          player.repeatState = player.repeatStates().ALL;
          player.repeat(player.repeatStates().ALL);
          break;
        case player.repeatStates().ALL:
          player.repeatState = player.repeatStates().NONE;
          player.repeat(player.repeatStates().NONE);
          break;
      }
    });

    elements.shuffle.addEventListener("click", () => {
      player.isShuffled ? player.unshuffle() : player.shuffle();
    });

    for (const artist of elements.artists) {
      artist.querySelector("button").addEventListener("dblclick", (event) => {
        const artist = {
          id: event.target.parentElement.id,
          ...event.target.parentElement.dataset,
        };
        console.log(artist);
      });
    }
    for (const album of elements.albums) {
      album.querySelector("button").addEventListener("dblclick", (event) => {
        const album = {
          id: event.target.parentElement.id,
          ...event.target.parentElement.dataset,
        };
        console.log(album);
      });
    }
    for (const el of elements.tracks) {
      el.querySelector("button").addEventListener("dblclick", async (event) => {
        const track = {
          id: event.target.parentElement.id,
          ...event.target.parentElement.dataset,
        };
        await player.setCurrentTrack(track);
        await player.play();

        console.log(track);
      });
    }
  },
};

const Player = {
  init: ({
    audioElement = document.getElementById("audio"),
    queue = new Array(),
    repeatOne = () => {
      console.log("repeatOne");
    },
    repeatAll = () => {
      console.log("repeatAll");
    },
    repeatNone = () => {
      console.log("repeatNone");
    },
    shuffle = () => {
      console.log("shuffle");
    },
    unshuffle = () => {
      console.log("unshuffle");
    },
    onTrackChange = (track) => {},
    onPlay = (track) => {},
    onPause = (track) => {},
  } = {}) => {
    const repeatStates = {
      ONE: "one",
      ALL: "all",
      NONE: "none",
    };

    const player = {
      isPlaying: () => !audioElement.paused,
      isShuffled: false,
      currentTrack: null,
      currentTrackIndex: 0,
      queue: queue,
      unshuffledQueue: queue,
      repeatState: repeatStates.NONE,
      repeatStates: function () {
        return repeatStates;
      },
      play: async function () {
        await audioElement.play();
        onPlay(this.currentTrack);
      },
      pause: async function () {
        await audioElement.pause();
        onPause(this.currentTrack);
      },
      next: async function () {
        switch (this.repeatState) {
          case this.repeatStates().NONE:
            if (this.currentTrackIndex >= this.queue.length - 1) {
              return null;
            }
            this.currentTrackIndex += 1;
            break;
          case this.repeatStates().ONE:
            await this.seekToBeginning();
            return null;
          case this.repeatStates().ALL:
            if (this.currentTrackIndex >= this.queue.length - 1) {
              this.currentTrackIndex = 0;
            } else {
              this.currentTrackIndex += 1;
            }
            break;
        }
        const track = this.queue[this.currentTrackIndex];
        await this.setCurrentTrack(track);
      },
      previous: async function () {
        this.currentTrackIndex > 0 ? (this.currentTrackIndex -= 1) : null;
        if (audioElement.currentTime > 0.3) {
          await this.seekToBeginning();
        } else {
          const track = this.queue[this.currentTrackIndex];
          await this.setCurrentTrack(track);
        }
      },
      repeat: function (kind) {
        switch (kind) {
          case repeatStates.ONE:
            repeatOne();
            break;
          case repeatStates.ALL:
            repeatAll();
            break;
          case repeatStates.NONE:
            repeatNone();
            break;
        }
      },
      shuffle: function () {
        this.setQueue(
          this.queue
            .map((value) => ({ value, sort: Math.random() }))
            .sort((a, b) => a.sort - b.sort)
            .map(({ value }) => value),
        );
        console.log(this.queue);
        shuffle();
      },
      unshuffle: function () {
        this.setQueue(this.unshuffledQueue);
        unshuffle();
      },
      seekToBeginning: async function () {
        if (this.isPlaying()) {
          audioElement.load();
          audioElement.onloadeddata = async () => {
            await audioElement.play();
          };
          audioElement.onloadedata = null;
        } else {
          audioElement.load();
        }
      },
      setCurrentTrack: async function (track) {
        if (this.isPlaying()) {
          audioElement.src = track.filePath;
          audioElement.onloadeddata = async () => {
            await audioElement.play();
          };
          audioElement.onloadedata = null;
        } else {
          audioElement.src = track.filePath;
        }
        onTrackChange(track);
      },
      setQueue: function (queue) {
        this.queue = queue;
      },
    };

    audioElement.addEventListener("ended", async () => {
      await player.next();
    });

    return player;
  },
};

// document.addEventListener("DOMContentLoaded", () => {
//   const queueElement = document.getElementById("queue");
//   const artists = document.getElementById("artists");
//   const albums = document.getElementById("albums");
//   const albumOptions = albums.querySelectorAll("li");
//   const tracks = document.getElementById("tracks");
//   const trackOptions = tracks.querySelectorAll("li");
//   const audio = document.getElementById("audio");
//   const playButton = document.getElementById("play");
//   const nextButton = document.getElementById("next");
//   const prevButton = document.getElementById("prev");
//   const nowPlaying = document.getElementById("now-playing");
//   const time = document.getElementById("time");
//   const repeatButton = document.getElementById("repeat");
//   const repeatStates = {
//     NO_REPEAT: 0,
//     REPEAT_ONE: 1,
//     REPEAT_ALL: 2,
//   };

//   let queue = new Map();
//   let queueIds = [];
//   let queueIndex = 0;
//   let repeatState = repeatStates.NO_REPEAT;
//   let isPlaying = false;
//   let currentTrack;

//   const updateQueueByDataset = ({ key, value }) => {
//     queue.clear();
//     queueIds = [];
//     queueIndex = 0;
//     trackOptions.forEach((trackOption) => {
//       if (trackOption.dataset[key] === value) {
//         const { id } = trackOption;
//         const { number, filePath, artistId, albumId, artist, album, name } =
//           trackOption.dataset;
//         queue.set(id, {
//           number,
//           filePath,
//           artistId,
//           albumId,
//           artist,
//           album,
//           name,
//         });
//         queueIds.push(id);
//       }
//     });
//     queueElement.innerHTML = "";
//     queue.forEach((track) => {
//       console.log(track);
//       const option = new Option(
//         `${track.artist} - ${track.album} - ${track.name}`,
//         track.filePath,
//       );
//       option.setAttribute("class", `bg-color-${(track.number % 36) + 1}`);
//       queueElement.appendChild(option);
//     });
//     currentTrack = queue.get(queueIds[queueIndex]);
//     audio.src = queue.get(queueIds[queueIndex]).filePath;
//   };

//   const updateQueue = (trackOptions) => {
//     queue.clear();
//     queueIds = [];
//     queueIndex = 0;
//     for (let trackOption of trackOptions) {
//       const { id } = trackOption;
//       const { number, filePath, artistId, albumId, artist, album, name } =
//         trackOption.dataset;
//       queue.set(id, {
//         number,
//         filePath,
//         artistId,
//         albumId,
//         artist,
//         album,
//         name,
//       });
//       queueIds.push(id);
//     }
//     queueElement.innerHTML = "";
//     queue.forEach((track) => {
//       console.log(track);
//       const option = new Option(
//         `${track.artist} - ${track.album} - ${track.name}`,
//         track.filePath,
//       );
//       option.setAttribute("class", `bg-color-${(track.number % 36) + 1}`);
//       queueElement.appendChild(option);
//     });
//     currentTrack = queue.get(queueIds[queueIndex]);
//     audio.src = queue.get(queueIds[queueIndex]).filePath;
//   };

//   const showHide = (elements, { key, value }) => {
//     elements.forEach((element) => {
//       if (element.dataset[key] === value) {
//         element.style.display = "block";
//         return;
//       }
//       if (element.style.display == "none") {
//         return;
//       }
//       element.style.display = "none";
//     });
//   };

//   const prev = () => {
//     switch (repeatState) {
//       case repeatStates.REPEAT_ONE:
//         break;
//       case repeatStates.REPEAT_ALL:
//         if (queueIndex > 0) {
//           queueIndex -= 1;
//         } else {
//           queueIndex = queueIds.length - 1;
//         }
//         break;
//       case repeatStates.NO_REPEAT:
//         if (queueIndex > 0) {
//           queueIndex -= 1;
//         }
//         break;
//     }
//     currentTrack = queue.get(queueIds[queueIndex]);
//     audio.src = queue.get(queueIds[queueIndex]).filePath;
//     if (isPlaying) {
//       play();
//     }
//   };
//   const next = () => {
//     switch (repeatState) {
//       case repeatStates.REPEAT_ONE:
//         break;
//       case repeatStates.REPEAT_ALL:
//         if (queueIndex < queueIds.length) {
//           queueIndex += 1;
//         } else {
//           queueIndex = 0;
//         }
//         break;
//       case repeatStates.NO_REPEAT:
//         if (queueIndex < queueIds.length) {
//           queueIndex += 1;
//         }
//         break;
//     }

//     audio.src = queue.get(queueIds[queueIndex]).filePath;
//     currentTrack = queue.get(queueIds[queueIndex]);

//     if (isPlaying) {
//       play();
//     }
//   };

//   const play = () => {
//     isPlaying = true;
//     playButton.innerText = "Pause";
//     nowPlaying.innerText = `${currentTrack.name} - ${currentTrack.artist} - ${currentTrack.album}`;
//     console.log(audio);
//     audio.play();
//   };
//   const pause = () => {
//     isPlaying = false;
//     playButton.innerText = "Play";
//     audio.pause();
//   };

//   const formatTime = (timeInSeconds) => {
//     const hours = String(
//       parseInt(Math.floor(timeInSeconds / (60 * 60))),
//     ).padStart(2, "0");
//     const minutes = String(parseInt(Math.floor(timeInSeconds / 60))).padStart(
//       2,
//       "0",
//     );
//     const seconds = String(parseInt(Math.floor(timeInSeconds % 60))).padStart(
//       2,
//       "0",
//     );

//     if (hours != "00") {
//       return `${hours}:${minutes}:${seconds}`;
//     }
//     return `${minutes}:${seconds}`;
//   };

//   audio.addEventListener("timeupdate", () => {
//     time.innerText = `${formatTime(audio.currentTime)} ${formatTime(audio.duration)}`;
//   });
//   repeatButton.addEventListener("click", () => {
//     repeatState = (repeatState + 1) % Object.keys(repeatStates).length;

//     switch (repeatState) {
//       case repeatStates.REPEAT_ONE:
//         repeatButton.innerText = "repeat (1)";
//         break;
//       case repeatStates.REPEAT_ALL:
//         repeatButton.innerText = "repeat all";
//         break;
//       case repeatStates.NO_REPEAT:
//         repeatButton.innerText = "repeat";
//         break;
//     }
//   });
//   artists.addEventListener("click", (event) => {
//     const artistId = event.target.id;
//     showHide(albumOptions, {
//       key: "artistId",
//       value: artistId,
//     });
//     showHide(trackOptions, {
//       key: "artistId",
//       value: artistId,
//     });
//   });
//   artists.addEventListener("dblclick", (event) => {
//     const artistId = event.target.id;
//     updateQueueByDataset({ key: "artistId", value: artistId });
//     play();
//   });
//   albums.addEventListener("click", (event) => {
//     const albumId = event.target.id;
//     showHide(trackOptions, {
//       key: "albumId",
//       value: albumId,
//     });
//   });
//   albums.addEventListener("dblclick", (event) => {
//     const albumId = event.target.id;
//     updateQueueByDataset({ key: "albumId", value: albumId });
//     play();
//   });
//   tracks.addEventListener("dblclick", (event) => {
//     updateQueue(event.target.parentElement.selectedOptions);
//     play();
//   });
//   nextButton.addEventListener("click", () => {
//     next();
//   });
//   audio.addEventListener("ended", () => {
//     next();
//   });
//   navigator.mediaSession.setActionHandler("nexttrack", () => {
//     next();
//   });
//   prevButton.addEventListener("click", () => {
//     prev();
//   });
//   playButton.addEventListener("click", () => {
//     isPlaying ? pause() : play();
//   });
// });
