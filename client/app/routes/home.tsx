import { Button } from "@fluentui/react-components";
import { useEffect, useState } from "react";
import { auth_api } from "~/shared/api";
import { _fetch } from "~/shared/util/fetch";

export default function Home() {
  const [displayName, setDisplayName] = useState();

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

  return (
    <div className="flex text-black">
      Home {displayName}
      <Button type="button" onClick={updateDisplayName}>
        update
      </Button>
    </div>
  );
}
