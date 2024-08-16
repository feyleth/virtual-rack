const BASE_URL = import.meta.env.VITE_BASE_URL;

export const watch_state = () => {
  return new EventSource(BASE_URL + "/api/state");
};
