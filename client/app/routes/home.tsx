import { Button } from "@fluentui/react-components";
import { useEffect, useState } from "react";
import { useFetchKeys } from "~/hooks/api/use-fetch-keys";
import { useFetchUserInfo } from "~/hooks/api/use-fetch-user-info";
import { useUpdateDisplayName } from "~/hooks/api/use-update-display-name";
import { useCheckHomeWebAvailable } from "~/hooks/use-check-home-web-available";
import { useCreateCredential } from "~/hooks/api/use-create-credential";
import { CredentialsResponse } from "~/models/credentials_model";
import { UserInfo } from "~/models/response_model";
import { _fetch } from "~/shared/util/fetch";
import { useDeleteCred } from "~/hooks/api/use-delete-cred";
import { useLogout } from "~/hooks/api/use-logout";
import { useUpdateRegisterKeyName } from "~/hooks/api/use-update-register-key-name";
import { LabelWithText } from "~/shared/components/home/label-with-text.component";
import { PasskeyItemList } from "~/shared/components/home/passkey-item.component";

export default function Home() {
  const [userInfo, setUserinfo] = useState<UserInfo>({
    displayname: "",
    id: "",
    username: "",
  });

  const [cred, setCred] = useState<Array<CredentialsResponse>>([]);
  const [availablePassKey, setAvailablePassKey] = useState<boolean>(true);

  const [createCred] = useCreateCredential();
  const { checkHomeWebAvailable } = useCheckHomeWebAvailable();

  const [fetchKeys] = useFetchKeys();
  const [fetchUserInfo] = useFetchUserInfo();
  const [upDisplayName] = useUpdateDisplayName();
  const [deletCred] = useDeleteCred();
  const [logout] = useLogout();
  const [upRegisterKey] = useUpdateRegisterKeyName();

  const getUserInfo = async () => {
    const data = await fetchUserInfo();
    if (data) {
      setUserinfo(data);
    }
  };

  const getKeys = async () => {
    const cred = await fetchKeys();
    if (cred) {
      setCred(cred);
    }
  };

  const updateDisplayName = async () => {
    const newDisplayName = prompt(
      "Enter new display name",
      userInfo.displayname
    );
    if (!newDisplayName) {
      return;
    }

    const user = await upDisplayName(newDisplayName);
    if (user) {
      setUserinfo(user);
    }
  };

  const registerCredential = async () => {
    const cred = await createCred();
    if (cred.data.id) {
      getKeys();
    }
  };

  const onDelete = async (credId: string) => {
    const onConfirm = confirm("Are you sure you want to delete key?");
    if (onConfirm) {
      const result = await deletCred(credId);
      if (result) {
        getKeys();
      }
    }
  };

  const onLogout = async () => {
    const result = confirm("Are you sure you want to logout?");
    if (result) {
      logout();
    }
  };

  const updateRegisterKeyName = async (credId: string, exName: string) => {
    const newName = prompt("Enter new cred name", exName);
    if (!newName) {
      return;
    }

    const res = await upRegisterKey({ credId, newName });
    if (res) {
      getKeys();
    }
  };

  const checkWebAvailable = async () => {
    const isAvailable = await checkHomeWebAvailable();
    setAvailablePassKey(isAvailable);
  };

  useEffect(() => {
    checkWebAvailable();
    getKeys();
    getUserInfo();
  }, []);

  return (
    <div className="text-black w-full h-full flex justify-center background-style">
      <div className="w-[480px] flex flex-col gap-8">
        <div className="h-[10%]" />
        {/** username */}
        <LabelWithText label="USER NAME" text={userInfo.username} />

        <LabelWithText
          label="DISPLAY NAME"
          text={userInfo.displayname}
          onClick={updateDisplayName}
        />

        <PasskeyItemList
          credItems={cred}
          renameItem={(id, name) => updateRegisterKeyName(id, name)}
          deleteItem={(id) => onDelete(id)}
        />

        {availablePassKey && (
          <Button type="button" size="large" onClick={registerCredential}>
            CREATE A PASSKEY
          </Button>
        )}

        <Button type="button" size="large" onClick={onLogout}>
          LOGOUT
        </Button>
      </div>
    </div>
  );
}
