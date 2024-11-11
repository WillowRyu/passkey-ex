const _API_URL = "http://localhost:3000/api";

export const auth_api = {
  username: `${_API_URL}/auth/username`,
  password: `${_API_URL}/auth/password`,
  userinfo: `${_API_URL}/auth/userinfo`,
  update_dispaly_name: `${_API_URL}/auth/updateDisplayName`,
  getkeys: `${_API_URL}/auth/getKeys`,
  registerRequest: `${_API_URL}/auth/registerRequest`,
  registerResponse: `${_API_URL}/auth/registerResponse`,
};
