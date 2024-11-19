export interface ResponseData<T> {
  data: T;
  message?: string;
}

export type UserInfo = {
  displayname: string;
  id: string;
  username: string;
};
