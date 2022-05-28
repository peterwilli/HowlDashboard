"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports["default"] = void 0;

require("./global.styl");

require("./themes/default/global.styl");

var _App = _interopRequireDefault(require("./App.svelte"));

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { "default": obj }; }

var app = new _App["default"]({
  target: document.body
});
var _default = app;
exports["default"] = _default;