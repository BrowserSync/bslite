import test from 'ava'

import { start } from '../index.js'

test('start is fn', (t) => {
  t.is(typeof start, "function")
})
