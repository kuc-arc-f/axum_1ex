const ApiUtil = {

  post: async function(path: string, values: any){
    try{
      values.external_api_path = path;
      const response = await fetch('/api/post_send', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(values),
      });
      if (!response.ok) {
        throw new Error('Failed response.ok= NG');
      }
      const json = await response.json();
      //console.log(json)
      if(json.ret !== 200){
        throw new Error("!Error, ret <> 200")
      }
      return json.data;
      //const text = await response.text();
      //console.log(text)
    }catch(e){
      console.error(e)
      throw new Error("Error, post")
    }

  },

  get: async function(path: string, values: any){
    try{
      values.external_api_path = path;
      const response = await fetch('/api/get_send', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(values),
      });
      if (!response.ok) {
        throw new Error('Failed response.ok= NG');
      }
      const json = await response.json();
      //console.log(json)
      if(json.ret !== 200){
        throw new Error("!Error, ret <> 200")
      }
      return json.data;
    }catch(e){
      console.error(e)
      throw new Error("Error, post")
    }

  },
}
export default ApiUtil;