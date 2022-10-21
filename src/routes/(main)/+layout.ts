import type { LayoutLoad } from './$types';

export const ssr = false;
export const prerender = false;

export const load: LayoutLoad = ({ params, url }) => {
  if (params.channel) {
    return {
      channel: params.channel,
      character: "",
      people: false
    }
  } else if (params.character) {
    return {
      character: params.character,
      people: false,
      channel: ""
    }
  } else {
    return {
      people: true,
      character: "",
      channel: ""
    }
  }
}
