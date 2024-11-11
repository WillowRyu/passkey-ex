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

  useEffect(() => {
    _fetch(auth_api.userinfo, "").then((res) => {
      console.log(res, "in home");
      setDisplayName(res?.data?.displayname);
    });
  }, []);

  const updateDisplayName = () => {
    _fetch(auth_api.update_dispaly_name, {
      display_name: "new name",
    }).then((res) => {
      if (res?.data?.displayname) {
        return setDisplayName(res.data.displayname);
      }

      alert(res?.message);
    });
  };

  const registerCredential = () => {
    createCred();
  };

  useEffect(() => {
    _fetch(auth_api.getkeys, "").then((res) => {
      if (res.data) {
        return setCred(res.data);
      }
      alert(res?.message);
    });
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
          return <span key={cred.id}>{cred.name}</span>;
        })}

        {cred.length === 0 && <span>no keys</span>}
      </div>
      <Button type="button" onClick={registerCredential}>
        register credential
      </Button>
    </div>
  );
}
