import { Paper } from '@mantine/core';
import { invoke } from '@tauri-apps/api/tauri';
import { useQuery } from '@tanstack/react-query';
import { TestFormProvider, TestFormInput } from './TestFormContext';
import TestFormFields from './TestFormFields';

type Tag = {
  id: string;
  name: string;
  kind: string;
  color: string;
};

const TagBox = ({ tag }: { tag: Tag }) => {
  const { id, name, kind, color } = tag;
  return (
    <div
      style={{
        width: '80%',
        padding: '10px',
        borderRadius: '5px',
        border: '1px solid red',
      }}
    >
      <div>Id: {id}</div>
      <div>Name: {name}</div>
      <div>Kind: {kind}</div>
      <div>Color: {color}</div>
    </div>
  );
};

const useGetAllTag = () => {
  return useQuery({
    queryKey: ['todos'],
    queryFn: async () => {
      const res: any = await invoke('get_all_tag2', {});
      const tags: Tag[] = res.result.data;
      return tags;
    },
  });
};

const Test = () => {
  const { data } = useGetAllTag();

  // eslint-disable-next-line @typescript-eslint/no-shadow
  const handleSubmit = (data: TestFormInput) => {
    console.log(data);
  };

  return (
    <Paper shadow="xs" radius="md" p="md" mt="md">
      <div>this is a test page na</div>
      <TestFormProvider
        onSubmit={handleSubmit}
        initValue={{ name: 'sssss', price: 200 }}
      >
        <TestFormFields />
        <button type="submit">submit</button>
      </TestFormProvider>
      {data ? data.map((t) => <TagBox key={t.id} tag={t} />) : null}
    </Paper>
  );
};

export default Test;
