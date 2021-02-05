import { Vec2, Vec3 } from './vec.js';

import Default from './subfolder/index.js';

import('./dynamic.js').then((x) => {
  Deno.core.print("Dynamic import resolved " + x.e);
  Deno.core.print("\n");
});

Deno.core.print(Default);
Deno.core.print("\n");

Deno.core.print(JSON.stringify(Vec2));
Deno.core.print("\n");

Deno.core.print(JSON.stringify(Vec3));
Deno.core.print("\n");


