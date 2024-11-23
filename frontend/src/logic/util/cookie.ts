export function getCookie(
  key: string,
  cookie: string,
) {
  if (typeof key !== "string" || !key) {
    return null;
  }

  const reKey = new RegExp(
    `(?:^|; )${key.replace(/[.*+?^$|[\](){}\\-]/g, "\\$&")}(?:=([^;]*))?(?:;|$)`,
  );
  const match = reKey.exec(cookie);

  if (match === null) {
    return null;
  }

  return decodeURIComponent(match[1]);
}
