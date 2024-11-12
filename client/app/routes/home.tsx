import { Button } from "@fluentui/react-components";
import { useEffect, useState } from "react";
import { useCreateCredential } from "~/hooks/use-create-credential";
import { CredentialsResponse } from "~/models/credentials_model";
import { auth_api } from "~/shared/api";
import { _fetch } from "~/shared/util/fetch";

export default function Home() {
  const [displayName, setDisplayName] = useState();
  const [cred, setCred] = useState<Array<CredentialsResponse>>([]);

  const { createCred } = useCreateCredential();

  const fetchKeys = () => {
    _fetch(auth_api.getkeys).then((res) => {
      if (res.data) {
        return setCred(res.data);
      }
      alert(res?.message);
    });
  };

  const fetchUserInfo = () => {
    _fetch(auth_api.userinfo).then((res) => {
      setDisplayName(res?.data?.displayname);
    });
  };

  const updateDisplayName = () => {
    const newDisplayName = prompt("Enter new display name", displayName);

    if (!newDisplayName) {
      return;
    }

    _fetch(auth_api.update_dispaly_name, {
      display_name: newDisplayName,
    }).then((res) => {
      if (res?.data?.displayname) {
        return setDisplayName(res.data.displayname);
      }

      alert(res?.message);
    });
  };

  const registerCredential = async () => {
    createCred().then((res) => {
      if (res.data.id) {
        fetchKeys();
      }
    });
  };

  const onDelete = (credId: string) => {
    _fetch(`${auth_api.removeKey}?credId=${credId}`).then((res) => {
      if (res.status === "OK") {
        fetchKeys();
      }
    });
  };

  const updateRegisterKeyName = (credId: string, exName: string) => {
    const newName = prompt("Enter new cred name", exName);

    if (!newName) {
      return;
    }

    _fetch(auth_api.renameKey, {
      credId,
      newName,
    }).then((res) => {
      if (res.data) {
        fetchKeys();
      }
    });
  };

  useEffect(() => {
    fetchKeys();
    fetchUserInfo();
  }, []);

  return (
    <div className="flex text-black">
      Home {displayName}
      <Button type="button" onClick={updateDisplayName}>
        update displayname
      </Button>
      <Button type="button" onClick={() => console.log("create register")}>
        create register
      </Button>
      <div>
        {cred?.map((cred) => {
          return (
            <div key={cred.id}>
              <span key={cred.id}>{cred.name}</span>
              <Button onClick={() => onDelete(cred.id)}>Delete</Button>
              <Button onClick={() => updateRegisterKeyName(cred.id, cred.name)}>
                Rename
              </Button>
            </div>
          );
        })}

        {cred.length === 0 && <span>no keys</span>}
      </div>
      <Button type="button" onClick={registerCredential}>
        register credential
      </Button>
    </div>
  );
}
