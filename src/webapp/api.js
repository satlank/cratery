function apiMe() {
  return fetch("/api/v1/me").then((response) => {
    if (response.status !== 200) {
      throw response.text();
    } else {
      return response.json();
    }
  });
}

function apiOAuthLoginWithCode(code) {
  return fetch("/api/v1/oauth/code", { method: "POST", body: code }).then(
    (response) => {
      if (response.status !== 200) {
        throw response.text();
      } else {
        return response.json();
      }
    }
  );
}

function apiLogout() {
  return fetch("/api/v1/logout", {
    method: "POST",
  }).then((r) => r.text());
}

function apiGetTokens() {
  return fetch("/api/v1/tokens").then((response) => {
    if (response.status !== 200) {
      throw response.text();
    } else {
      return response.json();
    }
  });
}

function apiCreateToken(name, canWrite, canAdmin) {
  return fetch(`/api/v1/tokens?canWrite=${canWrite}&canAdmin=${canAdmin}`, {
    method: "PUT",
    body: name,
  }).then((response) => {
    if (response.status !== 200) {
      throw response.text();
    } else {
      return response.json();
    }
  });
}

function apiRevokeToken(token_id) {
  return fetch(`/api/v1/tokens/${token_id}`, { method: "DELETE" }).then(
    (response) => {
      if (response.status !== 200) {
        throw response.text();
      } else {
        return response.text();
      }
    }
  );
}

function apiGetUsers() {
  return fetch("/api/v1/users").then((response) => {
    if (response.status !== 200) {
      throw response.text();
    } else {
      return response.json();
    }
  });
}

function apiUpdateUser(user) {
  return fetch(`/api/v1/users/${btoa(user.email)}`, {
    method: "PATCH",
    body: JSON.stringify(user),
    headers: [["content-type", "application/json"]],
  }).then((response) => {
    if (response.status !== 200) {
      throw response.text();
    } else {
      return response.json();
    }
  });
}

function apiDeleteUser(email) {
  return fetch(`/api/v1/users/${btoa(email)}`, {
    method: "DELETE",
  }).then((response) => {
    if (response.status !== 200) {
      throw response.text();
    } else {
      return response.json();
    }
  });
}

function apiDeactivateUser(email) {
  return fetch(`/api/v1/users/${btoa(email)}/deactivate`, {
    method: "POST",
  }).then((response) => {
    if (response.status !== 200) {
      throw response.text();
    } else {
      return response.text();
    }
  });
}

function apiReactivateUser(email) {
  return fetch(`/api/v1/users/${btoa(email)}/reactivate`, {
    method: "POST",
  }).then((response) => {
    if (response.status !== 200) {
      throw response.text();
    } else {
      return response.text();
    }
  });
}

function apiLookupCrates(input) {
  return fetch("/api/v1/crates?q=" + encodeURIComponent(input))
    .then((response) => {
      if (response.status !== 200) {
        throw response.text();
      } else {
        return response.json();
      }
    })
    .then((response) => response.crates);
}

function apiGetCrate(crate) {
  return fetch(`/api/v1/crates/${crate}`).then((response) => {
    if (response.status !== 200) {
      throw response.text();
    } else {
      return response.json();
    }
  });
}

function getQueryParameters(queryString) {
  const regex = new RegExp("[\\?&]([a-zA-Z0-9_-]+)=([^&#]*)", "g");
  let match = null;
  let result = {};
  do {
    match = regex.exec(queryString);
    if (match !== null) {
      let name = match[1];
      let value = decodeURIComponent(match[2].replace(/\+/g, " "));
      result[name] = value;
    }
  } while (match !== null);
  return result;
}

function getDatePart(input, regexp) {
  const result = input.match(new RegExp(regexp, "g"));
  if (result === null) {
    return "";
  }
  if (result.length === 0) {
    return "";
  }
  return result.pop();
}

function toDate(date) {
  if (date instanceof Date) {
    return date;
  }
  let datePart = getDatePart(date, "[0-9]{4}-[0-9]{2}-[0-9]{2}");
  if (datePart.length === 0) {
    datePart = getDatePart(date, "[0-9]{4}/[0-9]{2}/[0-9]{2}").replace(
      "/",
      "-"
    );
  }
  let timePart = getDatePart(date, "[0-9]{2}:[0-9]{2}:[0-9]{2}");
  if (timePart.length === 0) {
    timePart = "00:00:00";
  }
  return new Date(`${datePart}T${timePart}Z`);
}

function serializeDateTime(date) {
  if (date === null) {
    return "";
  }
  date = toDate(date);
  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(
    date.getDate()
  )} ${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(
    date.getSeconds()
  )}`;
}

function pad(x) {
  if (x < 10) {
    return "0" + x;
  }
  return x.toString();
}
