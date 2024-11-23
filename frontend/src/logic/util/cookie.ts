function getCookie<T = string>(
  key: string,
  cookie: string,
): T | null {
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
  
  return match[1] as T | null;
}
