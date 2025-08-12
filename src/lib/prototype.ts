/* eslint-disable no-extend-native */
import { handleError } from '@tb-dev/vue';

Promise.prototype.err = function() {
  this.catch((err: unknown) => handleError(err));
};
