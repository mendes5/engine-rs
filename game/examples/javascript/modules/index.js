import { Vec2, Vec3 } from './vec.js';

import Default from './subfolder/index.js';
import { TextDecoder } from './text-encoding.js';

import('./dynamic.js').then((x) => {
  Deno.core.print("Dynamic import resolved " + x.e);
  Deno.core.print("\n");
});

const decoder = new TextDecoder();

Deno.core.print(decoder);
Deno.core.print("\n");

Deno.core.print(Default);
Deno.core.print("\n");

Deno.core.print(JSON.stringify(Vec2));
Deno.core.print("\n");

Deno.core.print(JSON.stringify(Vec3));
Deno.core.print("\n");


