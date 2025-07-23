import AgentUtil  from '../lib/AgentUtil';

// first-agent
export function firstAgent(inputText){
  try{
    let text = "";
    const items = [
      { text: "firstGetRandom 6", title: "Step1: サイコロを振ります。" + "\n"} , 
      { text: "firstGetDate", title: "Step2: 現在の 日付を返します。" + "\n"} , 
      { text: "firstGetTime", title: "Step3: 現在の 時間を返します。" + "\n"} , 
    ];

    return items;
  }catch(e){
    console.error(e);
    throw new Error('Error , firstAgent');
  }
}

