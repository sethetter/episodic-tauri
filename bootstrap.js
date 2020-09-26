// import './static/style.scss';
import http from 'tauri/api/http';

import("./pkg").then(module => {
  window.http = http;
  module.run_app();
});
