import { Vec2, Vec3 } from './vec.js';

import Default from './subfolder/index.js';

import('./dynamic.js').then((x) => {
  runtime.print("Dynamic import resolved " + x.date);
  runtime.print("\n");
});

runtime.print(Default);
runtime.print("\n");

runtime.print(JSON.stringify(Vec2));
runtime.print("\n");

runtime.print(JSON.stringify(Vec3));
runtime.print("\n");


