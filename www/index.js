
import init, { World } from "snake_game";

init().then(_ => {

  const world = World.new();
  console.log(world.width);
})
