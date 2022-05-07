<script lang="ts">
  import { invoke, convertFileSrc } from '@tauri-apps/api/tauri'
  import { appWindow } from '@tauri-apps/api/window'
  import { dialog } from '@tauri-apps/api'
	import UIkit from 'uikit'
	import Icons from 'uikit/dist/js/uikit-icons'
	import 'uikit/dist/css/uikit.css'
	import 'uikit/dist/css/uikit.min.css'

	UIkit.use(Icons);

  let dir = 'not_exist_folder';
  invoke('get_dir')
  .then((d) => {
    dir = d;
    get_audio_list(dir);
  })
  .catch((_) => change_folder());

  let info = {};
  const get_audio_list = (path) => invoke('get_audio_list', {path: dir}).then((_info) => info = _info);

  const change_folder = () => {
    dialog.open({directory: true, multiple: false, recursive: false})
    .then((selected_dir) => {
      dir = selected_dir.replaceAll('\\', '/') + '/';
      invoke('set_dir', {dir: dir});
      get_audio_list(dir);
    });
  };

  const record = (e) => {
    const target = e.target;
    info[decodeURI(target.src.split('/').pop())] = target.currentTime;
    invoke('record_audio_list', {'info': info});
  };

  const set_current_time = (e) => {
    const target = e.target;
    const time = info[decodeURI(target.src.split('/').pop())]
    target.currentTime = time;
  };

  appWindow.listen('tauri://close-requested', ({ event, payload }) => {
    const audios = document.getElementsByTagName('audio');
    Array.from(audios).forEach(item => info[decodeURI(item.src.split('/').pop())] = item.currentTime);

    invoke('record_audio_list', {'info': info})
    .then(()=>appWindow.close());
  });
</script>

<main>
  <div class="uk-container uk-container-xlarge">
    {dir}<button id="change_dir" class="uk-button uk-button-default uk-button-small" on:click={change_folder}>change</button>

    <ul class="uk-list uk-list-divider">
      {#each Object.entries(info) as [name, _]}
        <li>
          <audio controls src={convertFileSrc(dir)+name} on:loadeddata="{set_current_time}" on:pause={record}/>
          {name}
        </li>
      {/each}
    </ul>
  </div>
</main>

<style lang="scss">
  #change_dir {
    margin-left: 1rem;
  }
</style>
