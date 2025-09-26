document.addEventListener("DOMContentLoaded", () => {
  ui.init();
});

const ui = {
  init: () => {
    const elements = {
      artists: document.getElementById("artists"),
      albums: document.getElementById("albums"),
      tracks: document.getElementById("tracks"),
      queue: document.getElementById("queue"),
      play: document.getElementById("play"),
      next: document.getElementById("next"),
      previous: document.getElementById("prev"),
      repeat: document.getElementById("repeat"),
      shuffle: document.getElementById("shuffle"),
    };

    const player = Player.init();

    elements.play.addEventListener("click", () =>
      player.isPlaying ? player.pause() : player.play(),
    );

    elements.next.addEventListener("click", () => {
      player.next();
    });

    elements.previous.addEventListener("click", () => {
      player.previous();
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
  },
};

const Player = {
  init: ({
    play = () => {
      console.log("play");
    },
    pause = () => {
      console.log("pause");
    },
    next = () => {
      console.log("next");
    },
    previous = () => {
      console.log("previous");
    },
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
  } = {}) => {
    const repeatStates = {
      ONE: "one",
      ALL: "all",
      NONE: "none",
    };
    return {
      isPlaying: false,
      isShuffled: false,
      currentTrack: null,
      artists: new Map(),
      albums: new Map(),
      tracks: new Map(),
      queue: new Map(),
      repeatState: repeatStates.NONE,
      repeatStates: function () {
        return repeatStates;
      },
      play: function () {
        this.isPlaying = true;
        play();
      },
      pause: function () {
        this.isPlaying = false;
        pause();
      },
      next: function () {
        next();
      },
      previous: function () {
        previous();
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
        }
      },
      shuffle: function () {
        this.isShuffled = true;
        shuffle();
      },
      unshuffle: function () {
        this.isShuffled = false;
        unshuffle();
      },
    };
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
