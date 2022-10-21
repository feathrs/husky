import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = ({ url }) => {
  if (url.pathname === "/people") throw redirect(307, "/people/everyone");
  return {};
}
