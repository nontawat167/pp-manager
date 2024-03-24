import { Paper } from '@mantine/core';
import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { TestFormProvider, TestFormInput } from './TestFormContext';
import TestFormFields from './TestFormFields';

const Test = () => {
  const handleSubmit = (data: TestFormInput) => {
    console.log(data);
  };

  const test = async () => {
    const res: any = await invoke('get_all_tag', {});
    const tags: any[] = res.result.data;

    // eslint-disable-next-line no-restricted-syntax
    for (const tag of tags) {
      console.log(tag);
    }
  };

  useEffect(() => {
    test();
  }, []);

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
    </Paper>
  );
};

export default Test;
